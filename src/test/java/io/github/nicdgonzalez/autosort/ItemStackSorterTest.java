package io.github.nicdgonzalez.autosort;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.util.ArrayList;
import java.util.List;
import org.bukkit.Material;
import org.bukkit.inventory.ItemStack;
import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.mockbukkit.mockbukkit.MockBukkit;
import org.mockbukkit.mockbukkit.ServerMock;

public class ItemStackSorterTest {
	private ServerMock server;

	@BeforeEach
	public void setUp() {
		server = MockBukkit.mock();
	}

	@AfterEach
	public void tearDown() {
		MockBukkit.unmock();
	}

	@Test
	public void testCombinesSimilarStacks() {
		List<ItemStack> itemStacks = new ArrayList<>(
				List.of(ItemStack.of(Material.DIRT, 32), ItemStack.of(Material.DIRT, 32)));

		ItemStackSorter.sort(itemStacks);

		assertEquals(itemStacks, List.of(ItemStack.of(Material.DIRT, 64)));
	}

	@Test
	public void testRemovesEmptySpace() {
		List<ItemStack> itemStacks = new ArrayList<>(List.of(ItemStack.empty(), ItemStack.of(Material.DIRT, 32),
				ItemStack.empty(), ItemStack.of(Material.DIRT, 32)));

		ItemStackSorter.sort(itemStacks);

		assertEquals(itemStacks, List.of(ItemStack.of(Material.DIRT, 64), ItemStack.empty()));
	}

	@Test
	public void testFixIllegalStackSize() {
		List<ItemStack> itemStacks = new ArrayList<>(List.of(ItemStack.of(Material.DIRT, 96)));

		ItemStackSorter.sort(itemStacks);

		assertEquals(itemStacks, List.of(ItemStack.of(Material.DIRT, 64), ItemStack.of(Material.DIRT, 32)));
	}
}
