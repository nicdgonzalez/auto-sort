package io.github.nicdgonzalez.autosort;

import org.bukkit.Bukkit;
import org.bukkit.plugin.java.JavaPlugin;

import io.github.nicdgonzalez.autosort.listeners.InventoryClickListener;

/**
 * This class is responsible for registering our event listeners.
 */
public class AutoSort extends JavaPlugin {
    @Override
    public void onEnable() {
        Bukkit.getPluginManager().registerEvents(new InventoryClickListener(), this);
    }
}
