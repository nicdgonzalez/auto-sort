package io.github.nicdgonzalez.autosort;

import java.util.Optional;

import org.bukkit.Material;
import org.bukkit.inventory.ItemStack;
import org.bukkit.inventory.meta.ItemMeta;

import net.kyori.adventure.text.serializer.plain.PlainTextComponentSerializer;

/**
 * {@link ItemStack} that is known to be a sort-triggering item.
 */
public class SortItem {
    private final ItemStack itemStack;
    private final SortAction action;

    /**
     * Creates a new {@link ItemStack} that is known to be a sort-triggering item.
     *
     * @param itemStack Item stack that can trigger a sort request.
     * @param action    Type of sort this item stack can trigger.
     */
    private SortItem(ItemStack itemStack, SortAction action) {
        this.itemStack = itemStack;
        this.action = action;
    }

    public ItemStack itemStack() {
        return itemStack;
    }

    public SortAction action() {
        return action;
    }

    /**
     * Creates a new {@link SortItem} from an {@link ItemStack}.
     *
     * An item stack is considered a valid sort-triggering item if:
     *
     * - It is not {@code null}.
     * - {@link ItemStack#getType} returns {@link Material.PAPER}.
     * - Custom name contains {@code "Sort"}, {@code "Paper"}, or both.
     *
     * @param itemStack Item stack to check.
     *
     * @return {@link SortItem} if item stack is a valid sort-triggering item,
     *         else {@link Optional#empty()}.
     */
    public static Optional<SortItem> fromItemStack(ItemStack itemStack) {
        if (itemStack == null) {
            return Optional.empty();
        }

        if (itemStack.getType() != Material.PAPER) {
            return Optional.empty();
        }

        if (!itemStack.hasItemMeta()) {
            return Optional.empty();
        }

        ItemMeta itemMeta = itemStack.getItemMeta();

        if (!itemMeta.hasCustomName()) {
            return Optional.empty();
        }

        String customName = PlainTextComponentSerializer
                .plainText()
                .serialize(itemMeta.customName())
                .toLowerCase();

        SortAction action;
        boolean hasSort = customName.contains("sort");
        boolean hasPlayer = customName.contains("player");

        if (hasSort && hasPlayer) {
            action = SortAction.TOP_AND_BOTTOM;
        } else if (hasSort) {
            action = SortAction.TOP_ONLY;
        } else if (hasPlayer) {
            action = SortAction.BOTTOM_ONLY;
        } else {
            return Optional.empty();
        }

        return Optional.of(new SortItem(itemStack, action));
    }
}
