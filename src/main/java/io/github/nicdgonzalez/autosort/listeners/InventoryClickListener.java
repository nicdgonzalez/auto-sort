package io.github.nicdgonzalez.autosort.listeners;

import java.util.Optional;
import java.util.UUID;

import org.bukkit.Material;
import org.bukkit.entity.Player;
import org.bukkit.event.EventHandler;
import org.bukkit.event.Listener;
import org.bukkit.event.inventory.InventoryClickEvent;
import org.bukkit.event.inventory.InventoryCloseEvent;
import org.bukkit.inventory.Inventory;
import org.bukkit.inventory.ItemStack;

import io.github.nicdgonzalez.autosort.SortItem;
import io.github.nicdgonzalez.autosort.SortItemTracker;
import io.github.nicdgonzalez.autosort.SortItemTracker.SortItemMetadata;
import net.kyori.adventure.text.serializer.plain.PlainTextComponentSerializer;

public class InventoryClickListener implements Listener {
    private final SortItemTracker sortItemTracker = new SortItemTracker();

    /**
     * Runs when the player clicks a slot in the inventory.
     */
    @EventHandler
    public void onInventoryClick(InventoryClickEvent event) {
        if (!(event.getWhoClicked() instanceof Player player)) {
            return;
        }

        UUID playerId = player.getUniqueId();
        Optional<SortItemMetadata> sortItemMetadata = sortItemTracker.get(playerId);

        if (sortItemMetadata.isPresent()) {
            // Player is putting an item down.
            SortItemMetadata metadata = sortItemMetadata.get();
            assert metadata.sortItem().itemStack().isSimilar(event.getCursor());

            Inventory clickedInventory = event.getClickedInventory();

            if (clickedInventory == null) {
                player.sendMessage("Item was thrown?");

                // If the player throws the item out and is close enough to pick it back up,
                // without this call, the trigger would be in reverse (i.e., sort on pick up
                // instead of put down).
                sortItemTracker.remove(playerId);

                return;
            }

            boolean isSameSlot = clickedInventory.getType() == metadata.inventoryType()
                    && event.getSlot() == metadata.slot();

            if (isSameSlot) {
                player.sendMessage("trigger sort request");
                SortItem sortItem = metadata.sortItem();

                switch (sortItem.action()) {
                    case TOP -> {
                        // Sort top inventory
                    }
                    case BOTTOM -> {
                        // Sort bottom inventory
                    }
                    case BOTH -> {
                        // Sort top, then bottom inventory.
                    }
                    case NONE -> {
                        // Do nothing.
                    }
                }
            }

            sortItemTracker.remove(playerId);
        } else {
            // To trigger a sort request, the player has to pick up
            // the sort-triggering item and put it back down in the same slot.
            // This is only possible if the user left-clicks the item.
            if (!event.isLeftClick()) {
                return;
            }

            // If the player's cursor is not empty, assume they are swapping
            // items around to organize their inventory (including if
            // the sort-triggering item is picked up in the process).
            if (event.getCursor().getType() != Material.AIR) {
                return;
            }

            ItemStack itemPickedUp = event.getCurrentItem();
            Optional<SortItem> sortItem = SortItem.fromItemStack(itemPickedUp);

            if (sortItem.isEmpty()) {
                return;
            }

            Inventory clickedInventory = event.getClickedInventory();
            assert clickedInventory != null : "expected sort-triggering item to be inside of the inventory";

            sortItemTracker.put(
                    playerId,
                    new SortItemMetadata(clickedInventory.getType(), event.getSlot(), sortItem.get()));
        }
    }

    /**
     * Removes the player's entry from {@link SortItemTracker} when the inventory
     * is closed to avoid keeping stale data.
     */
    @EventHandler
    public void onInventoryClose(InventoryCloseEvent event) {
        sortItemTracker.remove(event.getPlayer().getUniqueId());
    }
}
