package io.github.nicdgonzalez.autosort;

import java.util.Comparator;
import java.util.List;

import org.bukkit.Material;
import org.bukkit.inventory.ItemStack;

public class ItemStackSorter {
    private static final Comparator<SortKey> COMPARATOR = Comparator
            .comparingInt(SortKey::category)
            .thenComparingInt(SortKey::family)
            .thenComparingInt(SortKey::shape)
            .thenComparingInt(SortKey::modifier);

    public static void sort(List<ItemStack> itemStacks) {
        combineItemStacks(itemStacks);
        splitItemStacks(itemStacks);
        sortItemStacks(itemStacks);
    }

    private static void combineItemStacks(List<ItemStack> itemStacks) {
        for (int i = 0; i < itemStacks.size(); ++i) {
            ItemStack current = itemStacks.get(i);
            assert current != null : "null item stacks were not filtered";

            // Search the rest of the list for similar item stacks.
            for (int j = i + 1; j < itemStacks.size(); ++j) {
                ItemStack other = itemStacks.get(j);

                if (!current.isSimilar(other)) {
                    continue;
                }

                int amount = current.getAmount() + other.getAmount();
                current.setAmount(amount);

                // Removing from the list causes following elements to shift back by one.
                // Adjust the index to ensure the shifted element is not skipped.
                itemStacks.remove(j--);
            }
        }
    }

    private static void splitItemStacks(List<ItemStack> itemStacks) {
        for (int i = 0; i < itemStacks.size(); ++i) {
            ItemStack itemStack = itemStacks.get(i);
            assert itemStack != null : "null item stacks were not filtered";

            int amount = itemStack.getAmount();
            int maxStackSize = itemStack.getMaxStackSize();

            if (amount <= maxStackSize) {
                continue;
            }

            itemStack.setAmount(maxStackSize);
            amount -= maxStackSize;

            while (amount > 0) {
                int nextStackSize = Math.min(maxStackSize, amount);

                ItemStack nextItemStack = itemStack.clone();
                nextItemStack.setAmount(nextStackSize);

                itemStacks.add(++i, nextItemStack);
                amount -= nextStackSize;
            }
        }
    }

    private static void sortItemStacks(List<ItemStack> itemStacks) {
        itemStacks.sort(Comparator.comparing(ItemStackSorter::getSortKey, COMPARATOR));
    }

    private static SortKey getSortKey(ItemStack itemStack) {
        Material material = itemStack.getType();
        // There are currently only 8 categories (see code in `./crates`). Larger values
        // gives the unlabeled item stack the lowest priority, placing it at the end of
        // the inventory list.
        return SortKey.byName(material.name()).orElse(new SortKey(999, 0, 0, 0));
    }
}
