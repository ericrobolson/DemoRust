#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * A container for FFI state.
 * This should be obscured as the game state can change over time.
 */
typedef struct State State;

/**
 * A handle for a string.
 */
typedef struct StringId {
  uint64_t id;
} StringId;

typedef struct Position {
  float x;
  float y;
} Position;

typedef struct ViewState {
  struct Position entity_positions[500];
  uint32_t active_entities;
} ViewState;

/**
 * A handle for a texture.
 */
typedef struct TextureId {
  uint64_t id;
} TextureId;

/**
 * Attempts to fetch a string with the given handle.
 * Returns null if no string was present.
 */
const uint8_t *cg_resource_fetch_string_utf8(const struct State *cg_game_state,
                                             struct StringId resource);

struct StringId cb_get_string_id(void);

struct ViewState *cb_render_view_state(struct State *cg_game_state);

/**
 * Creates a new game state.
 */
struct State *cg_new(void);

/**
 * Ticks the game state.
 */
void cg_tick(struct State *cg_game_state);

/**
 * Registers the given texture with the engine.
 * Provides a texture id for future usage.
 */
struct TextureId cg_resource_register_texture(struct State *cg_game_state,
                                              uint32_t img_width,
                                              uint32_t img_height);

/**
 * Drops the given texture from the engine.
 */
void cg_resource_drop_texture(struct State *cg_game_state, struct TextureId texture);
