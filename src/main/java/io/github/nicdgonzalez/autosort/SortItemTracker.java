package io.github.nicdgonzalez.autosort;

import java.util.HashMap;
import java.util.Map;
import java.util.Optional;
import java.util.UUID;

public class SortItemTracker {
    private final Map<UUID, SortItemMetadata> trackedItems = new HashMap<>();

    public Optional<SortItemMetadata> get(UUID playerId) {
        SortItemMetadata trackedItem = trackedItems.get(playerId);
        return Optional.ofNullable(trackedItem);
    }

    public Optional<SortItemMetadata> put(UUID playerId, SortItemMetadata sortItem) {
        SortItemMetadata trackedItem = trackedItems.put(playerId, sortItem);
        return Optional.ofNullable(trackedItem);
    }

    public Optional<SortItemMetadata> remove(UUID playerId) {
        SortItemMetadata trackedItem = trackedItems.remove(playerId);
        return Optional.ofNullable(trackedItem);
    }
}
