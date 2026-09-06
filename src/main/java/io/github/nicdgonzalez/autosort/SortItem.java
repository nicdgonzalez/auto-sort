package io.github.nicdgonzalez.autosort;

import java.util.Optional;

import org.bukkit.Material;
import org.bukkit.inventory.ItemStack;
import org.bukkit.inventory.meta.ItemMeta;

import net.kyori.adventure.text.serializer.plain.PlainTextComponentSerializer;

/**
 * An {@link ItemStack} that is known to be a sort-triggering item.
 */
public class SortItem {
    public enum Action {
        TOP,
        BOTTOM,
        BOTH,
        NONE,
    }

    private final ItemStack itemStack;
    private final Action action;

    /**
     * Creates a new {@link ItemStack} that is known to be a sort-triggering item.
     *
     * @param itemStack Item stack that can trigger a sort request.
     * @param action    Type of sort this item stack can trigger.
     */
    private SortItem(ItemStack itemStack, Action action) {
        this.itemStack = itemStack;
        this.action = action;
    }

    public ItemStack itemStack() {
        return itemStack;
    }

    public Action action() {
        return action;
    }

    /**
     * Creates a new {@link SortItem} from an {@link ItemStack}.
     *
     * An item stack is considered a valid sort-triggering item if it is not
     * {@code null}, its {@link Material} is {@link Material.PAPER}, and
     * its name contains {@code "Sort"}, {@code "Paper"}, or both.
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

        String customName = PlainTextComponentSerializer.plainText().serialize(itemMeta.customName());
        String customNameLower = customName.toLowerCase();

        Action action;
        boolean hasSort = customNameLower.contains("sort");
        boolean hasPlayer = customNameLower.contains("player");

        if (hasSort && hasPlayer) {
            action = Action.BOTH;
        } else if (hasSort) {
            action = Action.TOP;
        } else if (hasPlayer) {
            action = Action.BOTTOM;
        } else {
            return Optional.empty();
        }

        return Optional.of(new SortItem(itemStack, action));
    }
}
