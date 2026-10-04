package io.github.nicdgonzalez.autosort;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

import org.bukkit.event.inventory.InventoryType;
import org.bukkit.inventory.Inventory;
import org.bukkit.inventory.ItemStack;

public class InventorySorter {
	public static void sort(Inventory inventory, SortExclusion exclusion) {
		// This is the inventory _before_ the excluded item was placed back into
		// the inventory.
		//
		// The player has picked up the sort-triggering item and put it back down
		// in the same slot in the inventory, which started the sorting operation.
		//
		// Whichever inventory the item was placed in should be missing one item, so
		// make sure to add it back in or the player will lose their sorting item.

		ItemStack[] storageContents = inventory.getStorageContents();
		int startIndex = getStartIndex(inventory.getType());

		boolean inTargetInventory = exclusion.inventory().equals(inventory);

		// Create a copy of the storage contents with `exclusion` removed.
		List<ItemStack> sortedItemStacks = IntStream.range(startIndex, storageContents.length)
				.filter(slot -> !(inTargetInventory && slot == exclusion.slot()))
				.mapToObj(slot -> storageContents[slot]).filter(item -> item != null)
				.collect(Collectors.toCollection(ArrayList::new));

		// Sort our copy of the storage contents.
		ItemStackSorter.sort(sortedItemStacks);

		// Clear the existing storage contents.
		Arrays.fill(storageContents, startIndex, storageContents.length, null);

		// Append all of the sorted items into the storage sequentially.
		int i = startIndex;
		for (int j = 0; i < storageContents.length && j < sortedItemStacks.size(); ++j) {
			storageContents[i++] = sortedItemStacks.get(j);
		}

		if (inTargetInventory) {
			boolean inHotbar = exclusion.slot() >= 0 && exclusion.slot() < startIndex;

			if (inHotbar) {
				// The player's hotbar is left untouched and the excluded item is currently
				// being held by the player's cursor.
				assert storageContents[exclusion.slot()] == null;
				storageContents[exclusion.slot()] = exclusion.itemStack();
			} else {
				// The excluded item was somewhere within the target inventory, and is currently
				// being held by the player's cursor.
				assert i < storageContents.length;
				assert storageContents[i] == null;
				storageContents[i] = exclusion.itemStack();
			}
		} else {
			// The player triggered a sort request for the top inventory from
			// the bottom inventory (or vice versa).
			Inventory otherInventory = exclusion.inventory();
			ItemStack[] otherStorageContents = otherInventory.getStorageContents();

			// The other storage's contents were left untouched and the excluded item is
			// currently being held by the player's cursor.
			assert otherStorageContents[exclusion.slot()] == null;
			otherStorageContents[exclusion.slot()] = exclusion.itemStack();

			otherInventory.setStorageContents(otherStorageContents);
		}

		inventory.setStorageContents(storageContents);
	}

	private static int getStartIndex(InventoryType inventoryType) {
		// When sorting player inventories, skip the hotbar (slots 0 to 8).
		return inventoryType.equals(InventoryType.PLAYER) ? 9 : 0;
	}
}
