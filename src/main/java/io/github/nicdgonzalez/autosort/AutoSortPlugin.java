package io.github.nicdgonzalez.autosort;

import io.github.nicdgonzalez.autosort.listeners.InventoryClickListener;
import org.bukkit.Bukkit;
import org.bukkit.plugin.java.JavaPlugin;

public class AutoSortPlugin extends JavaPlugin {
	@Override
	public void onEnable() {
		Bukkit.getPluginManager().registerEvents(new InventoryClickListener(), this);
	}
}
