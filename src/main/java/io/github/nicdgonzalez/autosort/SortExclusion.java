package io.github.nicdgonzalez.autosort;

import org.bukkit.inventory.Inventory;
import org.bukkit.inventory.ItemStack;

public record SortExclusion(Inventory inventory, ItemStack itemStack, int slot) {
}
