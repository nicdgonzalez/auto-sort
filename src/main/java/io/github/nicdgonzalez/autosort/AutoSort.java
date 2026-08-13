package io.github.nicdgonzalez.autosort;

import java.util.Arrays;

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
    static final InventoryType[] SORTABLE_ITEMS = {
            InventoryType.CHEST,
            InventoryType.ENDER_CHEST,
            InventoryType.BARREL,
            InventoryType.SHULKER_BOX,
    };

    /** Item to click to trigger sorting. */
    static final Material TARGET_TYPE = Material.PAPER;
    /** Unique name that the item must have to trigger sorting. */
    static final Component TARGET_NAME = Component.text("Auto Sort");

    @Override
    public void onEnable() {
        Bukkit.getPluginManager().registerEvents(this, this);
    }

    /**
     * Checks if a player triggered a sort request.
     *
     * When the player picks up a specific item, it triggers a sort request. We use
     * a specific item as our trigger because it allows both Java /and/ Bedrock
     * players to trigger sorting for themselves without needing any client-side
     * modifications. The request can be handled entirely on the server.
     */
    @EventHandler
    public void onInventoryClick(InventoryClickEvent event) {
        if (!(event.getWhoClicked() instanceof Player)) {
            return;
        }

        Inventory inventory = event.getView().getTopInventory();
        InventoryType type = inventory.getType();

        if (!(Arrays.asList(SORTABLE_ITEMS).contains(type))) {
            return;
        }

        ItemStack item = event.getCurrentItem();

        if (!isAutoSortItem(item)) {
            return;
        }

        // We only sort the top inventory since personal inventories can be sorted
        // in many different ways depending on a player's preference.
        sortTopInventory(event.getView().getTopInventory());
    }

    /** Checks if `item` is the special auto-sorting item. */
    boolean isAutoSortItem(ItemStack item) {
        ItemMeta meta = item.getItemMeta();

        if (!meta.hasCustomName()) {
            return false;
        }

        return item.getType() == TARGET_TYPE && meta.customName().equals(TARGET_NAME);
    }

    /** Sorts all of the items in the given `inventory`. */
    void sortTopInventory(Inventory inventory) {
        // TODO: We might be in for a doozy...
    }
}
