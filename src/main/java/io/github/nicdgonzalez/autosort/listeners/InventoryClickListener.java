package io.github.nicdgonzalez.autosort.listeners;

import java.util.UUID;

import org.bukkit.Material;
import org.bukkit.entity.Player;
import org.bukkit.event.EventHandler;
import org.bukkit.event.Listener;
import org.bukkit.event.inventory.InventoryClickEvent;
import org.bukkit.event.inventory.InventoryCloseEvent;
import org.bukkit.inventory.Inventory;
import org.bukkit.inventory.ItemStack;

import io.github.nicdgonzalez.autosort.InventorySorter;
import io.github.nicdgonzalez.autosort.SortExclusion;
import io.github.nicdgonzalez.autosort.SortItem;
import io.github.nicdgonzalez.autosort.SortItemTracker;
import io.github.nicdgonzalez.autosort.SortItemTracker.SortItemMetadata;

public class InventoryClickListener implements Listener {
    private final SortItemTracker sortItemTracker = new SortItemTracker();

    @EventHandler
    public void onInventoryClick(InventoryClickEvent event) {
        if (!(event.getWhoClicked() instanceof Player player)) {
            return;
        }

        UUID playerId = player.getUniqueId();
        SortItemMetadata sortItemMetadata = sortItemTracker.get(playerId).orElse(null);

        if (sortItemMetadata != null) {
            assert event.getCursor().isSimilar(sortItemMetadata.sortItem().itemStack())
                    : "item stack under the cursor should be the same as in the tracker";

            Inventory clickedInventory = event.getClickedInventory();

            if (clickedInventory == null) {
                // If the player throws the item out and is close enough to pick it back up,
                // without this call, the trigger would be in reverse (i.e., sort on pick up
                // instead of put down).
                sortItemTracker.remove(playerId);
                return;
            }

            boolean isSameSlot = sortItemMetadata.inventory().getType() == clickedInventory.getType()
                    && event.getSlot() == sortItemMetadata.slot();

            if (isSameSlot) {
                SortExclusion exclusion = new SortExclusion(
                        event.getClickedInventory(),
                        sortItemMetadata.sortItem().itemStack(),
                        sortItemMetadata.slot());

                try {
                    switch (sortItemMetadata.sortItem().action()) {
                        case TOP_ONLY -> {
                            InventorySorter.sort(event.getView().getTopInventory(), exclusion);
                        }
                        case BOTTOM_ONLY -> {
                            InventorySorter.sort(event.getView().getBottomInventory(), exclusion);
                        }
                        case TOP_AND_BOTTOM -> {
                            InventorySorter.sort(event.getView().getTopInventory(), exclusion);
                            InventorySorter.sort(event.getView().getBottomInventory(), exclusion);
                        }
                        case NONE -> {
                        }
                    }
                } catch (Exception e) {
                    e.printStackTrace();
                }
            }

            sortItemTracker.remove(playerId);
        } else {
            // To trigger a sort request, the player has to pick up the sort-triggering item
            // and put it back down in the same slot. This is only possible if the user
            // left-clicks the item.
            if (!event.isLeftClick()) {
                return;
            }

            // If the player's cursor is not empty, assume they are swapping items around to
            // organize their inventory (including if the sort-triggering item is selected).
            if (event.getCursor().getType() != Material.AIR) {
                return;
            }

            ItemStack itemPickedUp = event.getCurrentItem();
            SortItem sortItem = SortItem.fromItemStack(itemPickedUp).orElse(null);

            if (sortItem == null) {
                return;
            }

            Inventory clickedInventory = event.getClickedInventory();
            assert clickedInventory != null : "SortItem was clicked from outside inventory view";

            SortItemMetadata metadata = new SortItemMetadata(
                    clickedInventory,
                    sortItem,
                    event.getSlot());

            sortItemTracker.put(playerId, metadata);
        }
    }

    /**
     * Cleans up the player's now stale data from {@link SortItemTracker}.
     */
    @EventHandler
    public void onInventoryClose(InventoryCloseEvent event) {
        sortItemTracker.remove(event.getPlayer().getUniqueId());
    }
}
