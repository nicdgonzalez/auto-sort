package io.github.nicdgonzalez.autosort;

import java.util.List;
import java.util.Optional;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.Set;
import java.util.stream.Collectors;

import org.bukkit.Bukkit;
import org.bukkit.Material;
import org.bukkit.entity.Player;
import org.bukkit.event.EventHandler;
import org.bukkit.event.Listener;
import org.bukkit.event.inventory.InventoryClickEvent;
import org.bukkit.event.inventory.InventoryType;
import org.bukkit.inventory.Inventory;
import org.bukkit.inventory.ItemStack;
import org.bukkit.inventory.meta.ItemMeta;
import org.bukkit.plugin.java.JavaPlugin;

import net.kyori.adventure.text.Component;

public class AutoSort extends JavaPlugin implements Listener {
    /** Inventory types that can be sorted. */
    private static final Set<InventoryType> SORTABLE_ITEMS = Set.of(
            InventoryType.CHEST,
            InventoryType.ENDER_CHEST,
            InventoryType.BARREL,
            InventoryType.SHULKER_BOX);

    /** Item to click to trigger sorting. */
    private static final Material TARGET_TYPE = Material.PAPER;
    /** Unique name that the item must have to trigger sorting. */
    private static final Component TARGET_NAME = Component.text("Sort");

    /** For grouping and sorting items based on `SortKey`. */
    private static final Comparator<SortKey> SORT_KEY_COMPARATOR = Comparator
            .comparingInt(SortKey::category)
            .thenComparingInt(SortKey::family)
            .thenComparingInt(SortKey::variant);

    @Override
    public void onEnable() {
        Bukkit.getPluginManager().registerEvents(this, this);
    }

    /**
     * Checks if a player triggered a sort request.
     *
     * When the player picks up a specific item, it triggers a sort request. We use
     * an item as our trigger because it allows both Java /and/ Bedrock players to
     * trigger sorting without needing any client-side modifications. The request
     * is handled entirely on the server.
     */
    @EventHandler
    public void onInventoryClick(InventoryClickEvent event) {
        if (!(event.getWhoClicked() instanceof Player)) {
            return;
        }

        Inventory inventory = event.getView().getTopInventory();
        InventoryType type = inventory.getType();

        if (!SORTABLE_ITEMS.contains(type)) {
            return;
        }

        ItemStack item = event.getCurrentItem();

        if (!isAutoSortItem(item)) {
            return;
        }

        sortTopInventory(event.getView().getTopInventory());
    }

    /** Checks if `item` is the special auto-sorting item. */
    private boolean isAutoSortItem(ItemStack item) {
        if (!item.hasItemMeta()) {
            return false;
        }

        ItemMeta meta = item.getItemMeta();

        if (!meta.hasCustomName()) {
            return false;
        }

        return item.getType() == TARGET_TYPE && meta.customName().equals(TARGET_NAME);
    }

    /** Sorts all of the items in the given `inventory`. */
    private void sortTopInventory(Inventory inventory) {
        List<ItemStack> items = Arrays.stream(inventory.getContents())
                .filter(item -> item != null && !item.getType().equals(Material.AIR))
                .collect(Collectors.toCollection(ArrayList::new));

        items = organizeStacks(items);
        items.sort(Comparator.comparing(item -> getSortKey(item.getType()), SORT_KEY_COMPARATOR));

        ItemStack[] contents = new ItemStack[inventory.getSize()];
        assert items.size() <= contents.length : "sorting shouldn't create items";

        for (int i = 0; i < items.size(); ++i) {
            contents[i] = items.get(i);
        }

        inventory.setContents(contents);
    }

    /** Combines unfinished stacks into multiple max-sized stacks. */
    private List<ItemStack> organizeStacks(List<ItemStack> items) {
        items = combineStacks(items);
        items = splitStacks(items);
        return items;
    }

    // TODO: Combine `combineStacks` and `splitStacks` into one function call
    // to prevent accidentally calling one and not the other.

    /**
     * Combines similar stacks into a single stack that may exceed stack limit.
     *
     * Call `splitStacks` on the result before setting this back into the inventory.
     */
    private List<ItemStack> combineStacks(List<ItemStack> items) {
        List<ItemStack> combined = new ArrayList<>();

        for (ItemStack item : items) {
            boolean merged = false;

            for (ItemStack existing : combined) {
                if (!existing.isSimilar(item)) {
                    continue;
                }

                existing.setAmount(existing.getAmount() + item.getAmount());
                merged = true;
                break;
            }

            if (!merged) {
                combined.add(item.clone());
            }
        }

        return combined;
    }

    /** Separates combined stacks into multiple max-sized stacks. */
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
            this.getLogger().warning(String.format("Failed to sort item: %s", material.name()));
        }

        return key.orElse(new SortKey(99, 0, material.ordinal()));
    }
}
