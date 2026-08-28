package io.github.nicdgonzalez.autosort;

import org.bukkit.Bukkit;
import org.bukkit.plugin.java.JavaPlugin;

public class AutoSort extends JavaPlugin {
    @Override
    public void onEnable() {
        Bukkit.getPluginManager().registerEvents(new InventoryClickListener(this), this);
    }
}
