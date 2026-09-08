package io.github.nicdgonzalez.autosort;

import org.bukkit.inventory.Inventory;

public record SortItemMetadata(Inventory inventory, SortItem sortItem, int slot) {
}
