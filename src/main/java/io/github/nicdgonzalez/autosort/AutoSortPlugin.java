package io.github.nicdgonzalez.autosort;

import org.bukkit.Bukkit;
import org.bukkit.plugin.java.JavaPlugin;

import io.github.nicdgonzalez.autosort.listeners.InventoryClickListener;

public class AutoSortPlugin extends JavaPlugin {
    @Override
    public void onEnable() {
        Bukkit.getPluginManager().registerEvents(new InventoryClickListener(), this);
    }
}
