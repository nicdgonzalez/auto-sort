package io.github.nicdgonzalez.autosort;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import java.util.Set;
import java.util.UUID;

import org.bukkit.Material;
import org.bukkit.entity.Player;
import org.bukkit.event.EventHandler;
import org.bukkit.event.Listener;
import org.bukkit.event.inventory.InventoryClickEvent;
import org.bukkit.event.inventory.InventoryCloseEvent;
import org.bukkit.event.inventory.InventoryType;
import org.bukkit.inventory.Inventory;
import org.bukkit.inventory.ItemStack;
import org.bukkit.inventory.meta.ItemMeta;

import net.kyori.adventure.text.serializer.plain.PlainTextComponentSerializer;

public class InventoryClickListener implements Listener {

    /** Inventory types that can be sorted. */
    private static final Set<InventoryType> SORTABLE_ITEMS = Set.of(
            InventoryType.CHEST,
            InventoryType.ENDER_CHEST,
            InventoryType.BARREL,
            InventoryType.SHULKER_BOX,
            InventoryType.PLAYER);

    /** Item used to trigger sorting. */
    private static final Material TARGET_TYPE = Material.PAPER;

    /** For grouping and sorting items based on SortKey. */
    private static final Comparator<SortKey> SORT_KEY_COMPARATOR = Comparator
            .comparingInt(SortKey::category)
            .thenComparingInt(SortKey::family)
            .thenComparingInt(SortKey::shape)
            .thenComparingInt(SortKey::modifier);

    /**
     * Tracks a player who has picked up an AutoSort item and is waiting
     * to see where they put it down.
     */
    private final Map<UUID, PendingSort> pendingSorts = new HashMap<>();

    // TODO: Remove this when we are no longer depending on the plugin's logger.
    private final AutoSort plugin;

    public InventoryClickListener(AutoSort plugin) {
        this.plugin = plugin;
    }

    /**
     * Represents an AutoSort item that a player has picked up.
     *
     * The inventory and slot identify the exact location from which the
     * item was picked up.
     */
    private record PendingSort(
            Inventory inventory,
            int slot,
            ItemStack item) {
    }

    /**
     * Represents what the clicked AutoSort paper should sort.
     */
    private enum SortAction {
        TOP,
        PLAYER,
        BOTH,
        NONE
    }

    /**
     * Checks if a player triggered a sort request.
     *
     * A player must pick up the AutoSort item and put it back down in the
     * exact same slot. Moving it somewhere else behaves normally.
     */
    @EventHandler
    public void onInventoryClick(InventoryClickEvent event) {
        if (!(event.getWhoClicked() instanceof Player player)) {
            return;
        }

        Inventory inventoryTop = event.getView().getTopInventory();
        Inventory inventoryBottom = event.getView().getBottomInventory();

        InventoryType typeTop = inventoryTop.getType();
        InventoryType typeBottom = inventoryBottom.getType();

        if (!SORTABLE_ITEMS.contains(typeTop)
                && !SORTABLE_ITEMS.contains(typeBottom)) {
            return;
        }

        UUID playerId = player.getUniqueId();
        PendingSort pending = pendingSorts.get(playerId);

        /*
         * The player previously picked up the AutoSort item.
         *
         * Determine whether this click puts it back in the exact same
         * inventory and slot.
         */
        if (pending != null) {
            ItemStack cursor = event.getCursor();

            /*
             * Only treat the cursor as the pending AutoSort item if it is
             * actually the same type of AutoSort paper that was picked up.
             *
             * Do NOT use isAutoSortItem(cursor) here. That only tells us that
             * the cursor contains SOME AutoSort paper (Sort, Player, or
             * Sort+Player), which can cause two different papers to interfere
             * with each other's pending state.
             */
            if (cursor != null && cursor.isSimilar(pending.item())) {
                boolean sameSlot = event.getClickedInventory() == pending.inventory()
                        && event.getSlot() == pending.slot();

                if (sameSlot) {
                    event.setCancelled(true);
                    pendingSorts.remove(playerId);

                    /*
                     * Since the event is cancelled, Bukkit will not put the
                     * cursor item back into the clicked slot for us.
                     *
                     * Restore the exact paper that was originally picked up.
                     */
                    pending.inventory().setItem(
                            pending.slot(),
                            pending.item().clone());

                    /*
                     * The paper has now been returned to its original slot.
                     * Remove it from the cursor so there is exactly one copy.
                     */
                    player.setItemOnCursor(new ItemStack(Material.AIR));

                    /*
                     * Determine which inventory/inventories this specific
                     * paper should sort.
                     */
                    SortAction action = getSortAction(pending.item());

                    switch (action) {
                        case TOP -> sortInventory(
                                inventoryTop,
                                pending.item(),
                                pending.inventory(),
                                pending.slot());

                        case PLAYER -> sortInventory(
                                inventoryBottom,
                                pending.item(),
                                pending.inventory(),
                                pending.slot());

                        case BOTH -> {
                            /*
                             * Sort the top and bottom independently.
                             *
                             * The source inventory and source slot remain
                             * unchanged, so the trigger paper cannot migrate
                             * between inventories.
                             */
                            sortInventory(
                                    inventoryTop,
                                    pending.item(),
                                    pending.inventory(),
                                    pending.slot());

                            sortInventory(
                                    inventoryBottom,
                                    pending.item(),
                                    pending.inventory(),
                                    pending.slot());
                        }

                        case NONE -> {
                            // Should not happen because the pending item was
                            // originally verified by isAutoSortItem().
                        }
                    }

                    return;
                }

                /*
                 * The cursor contains the same AutoSort paper, but the player
                 * is putting it somewhere other than its original slot.
                 *
                 * This is a normal inventory move, so stop tracking it.
                 */
                pendingSorts.remove(playerId);
                return;
            }

            /*
             * Something other than the specific pending paper is now on the
             * cursor.
             *
             * Do not let another AutoSort paper (Sort, Player, or Sort+Player)
             * satisfy the pending state.
             */
            pendingSorts.remove(playerId);
        }

        ItemStack item = event.getCurrentItem();

        if (!isAutoSortItem(item)) {
            return;
        }

        /*
         * Only start tracking when the player is actually picking up
         * the AutoSort item with an empty cursor.
         */
        if (event.getCursor().getType() != Material.AIR) {
            return;
        }

        /*
         * Ignore shift-clicks. They don't represent the pickup/put-down
         * gesture we're using as the sort button.
         */
        if (event.isShiftClick()) {
            return;
        }

        /*
         * Store the exact source inventory and slot.
         *
         * We intentionally do NOT store only the target inventory because
         * the paper may be in the player's inventory while sorting a chest.
         */
        pendingSorts.put(
                playerId,
                new PendingSort(
                        event.getClickedInventory(),
                        event.getSlot(),
                        item.clone()));
    }

    /**
     * Clear pending state when a player closes an inventory.
     */
    @EventHandler
    public void onInventoryClose(InventoryCloseEvent event) {
        pendingSorts.remove(event.getPlayer().getUniqueId());
    }

    /**
     * Determines what action an AutoSort paper represents.
     *
     * Names are matched case-insensitively.
     *
     * Supported names:
     * Sort -> top inventory
     * Player -> player inventory
     * Sort+Player -> both
     */
    private SortAction getSortAction(ItemStack item) {
        if (item == null || !item.hasItemMeta()) {
            return SortAction.NONE;
        }

        ItemMeta meta = item.getItemMeta();

        if (!meta.hasCustomName()) {
            return SortAction.NONE;
        }

        String customName = PlainTextComponentSerializer.plainText()
                .serialize(meta.customName())
                .toLowerCase();

        boolean sort = customName.contains("sort");
        boolean player = customName.contains("player");

        if (sort && player) {
            return SortAction.BOTH;
        }

        if (sort) {
            return SortAction.TOP;
        }

        if (player) {
            return SortAction.PLAYER;
        }

        return SortAction.NONE;
    }

    /**
     * Checks if item is the special AutoSort item.
     */
    private boolean isAutoSortItem(ItemStack item) {
        if (item == null || item.getType() != TARGET_TYPE) {
            return false;
        }

        if (!item.hasItemMeta()) {
            return false;
        }

        ItemMeta meta = item.getItemMeta();

        if (!meta.hasCustomName()) {
            return false;
        }

        String customName = PlainTextComponentSerializer.plainText()
                .serialize(meta.customName())
                .toLowerCase();

        return customName.contains("sort")
                || customName.contains("player");
    }

    /**
     * Sorts all sortable items in the given inventory.
     *
     * For a player inventory, hotbar slots 0-8 are left completely
     * untouched.
     *
     * The source inventory and slot identify where the trigger paper
     * originally came from. The paper is only restored into the inventory
     * being sorted when that inventory is also its source inventory.
     *
     * This is the important distinction that prevents a "Sort" paper
     * sitting in the player's hotbar from being inserted into a chest.
     */
    private void sortInventory(
            Inventory inventory,
            ItemStack extraItem,
            Inventory sourceInventory,
            int sourceSlot) {

        boolean isPlayerInventory = inventory.getType() == InventoryType.PLAYER;

        ItemStack[] contents = isPlayerInventory
                ? inventory.getStorageContents()
                : inventory.getContents();

        /*
         * Player inventory:
         *
         * 0-8 = hotbar
         * 9+ = main inventory
         *
         * The hotbar is never modified by sorting.
         */
        int startSlot = isPlayerInventory ? 9 : 0;

        /*
         * The trigger paper is the ONLY AutoSort paper that is protected
         * from sorting.
         *
         * Other AutoSort papers are treated exactly like normal items and
         * are allowed to participate in the sort.
         *
         * We only need to exclude the source slot when the source inventory
         * is the inventory currently being sorted and the source slot is
         * actually inside the sortable area.
         */
        boolean triggerItemIsInSortTarget = sourceInventory == inventory
                && sourceSlot >= startSlot
                && sourceSlot < contents.length;

        /*
         * Collect all items except the specific paper that triggered the
         * sort.
         *
         * IMPORTANT:
         * Do not use isAutoSortItem() here. Other AutoSort papers should
         * remain part of the sortable contents.
         */
        List<ItemStack> items = new ArrayList<>();

        for (int slot = startSlot; slot < contents.length; ++slot) {

            /*
             * Leave the triggering paper out of the sorting operation.
             * It will be restored to this exact slot afterward.
             */
            if (triggerItemIsInSortTarget && slot == sourceSlot) {
                continue;
            }

            ItemStack item = contents[slot];

            if (item == null || item.getType() == Material.AIR) {
                continue;
            }

            /*
             * Every other item—including other Sort, Player, and
             * Sort+Player papers—is allowed to be sorted normally.
             */
            items.add(item.clone());
        }

        /*
         * Combine and split stacks before sorting.
         */
        items = organizeStacks(items);

        /*
         * Sort according to the existing SortKey implementation.
         *
         * Other AutoSort papers are now included here and therefore follow
         * the same sorting rules as every other item.
         */
        items.sort(Comparator.comparing(
                item -> getSortKey(item.getType()),
                SORT_KEY_COMPARATOR));

        /*
         * Clear ONLY the sortable portion.
         *
         * For player inventories, slots 0-8 (the hotbar) remain untouched.
         */
        Arrays.fill(contents, startSlot, contents.length, null);

        /*
         * Restore the ONE paper that triggered the sort to its exact
         * original position.
         */
        if (triggerItemIsInSortTarget) {
            contents[sourceSlot] = extraItem.clone();
        }

        /*
         * Fill the sorted items into every remaining available slot.
         *
         * The triggering paper's slot is skipped so that it cannot be
         * overwritten by a sorted item.
         */
        int itemIndex = 0;

        for (int slot = startSlot; slot < contents.length && itemIndex < items.size(); ++slot) {

            if (triggerItemIsInSortTarget && slot == sourceSlot) {
                continue;
            }

            contents[slot] = items.get(itemIndex++);
        }

        /*
         * Write the modified contents back.
         */
        if (isPlayerInventory) {
            inventory.setStorageContents(contents);
        } else {
            inventory.setContents(contents);
        }
    }

    /**
     * Combines and splits stacks before sorting.
     */
    private List<ItemStack> organizeStacks(List<ItemStack> items) {
        items = combineStacks(items);
        items = splitStacks(items);

        return items;
    }

    /**
     * Combines similar stacks into a single stack that may exceed
     * the normal stack limit.
     *
     * splitStacks() is called afterward to restore valid stack sizes.
     */
    private List<ItemStack> combineStacks(List<ItemStack> items) {
        List<ItemStack> combined = new ArrayList<>();

        for (ItemStack item : items) {
            boolean merged = false;

            for (ItemStack existing : combined) {
                if (!existing.isSimilar(item)) {
                    continue;
                }

                existing.setAmount(
                        existing.getAmount() + item.getAmount());

                merged = true;
                break;
            }

            if (!merged) {
                combined.add(item.clone());
            }
        }

        return combined;
    }

    /**
     * Separates combined stacks into multiple max-sized stacks.
     */
    private List<ItemStack> splitStacks(List<ItemStack> items) {
        List<ItemStack> result = new ArrayList<>();

        for (ItemStack item : items) {
            int amount = item.getAmount();
            int max = item.getMaxStackSize();

            while (amount > 0) {
                int stackSize = Math.min(amount, max);

                ItemStack stack = item.clone();
                stack.setAmount(stackSize);

                result.add(stack);

                amount -= stackSize;
            }
        }

        return result;
    }

    /** Determines how items get sorted. */
    private SortKey getSortKey(Material material) {
        Optional<SortKey> key = SortKey.byName(material.name());

        if (key.isEmpty()) {
            this.plugin.getLogger().warning(
                    String.format(
                            "Failed to sort item: %s",
                            material.name()));
        }

        return key.orElse(
                new SortKey(99, 0, 0, material.ordinal()));
    }
}
