#ifndef GOSUB_API_H
#define GOSUB_API_H

#include <stdbool.h>
#include <stddef.h> // for NULL (which is basically just 0... but more clear)
#include <stdint.h>
#include <stdlib.h>

extern void *render_tree_init();
extern void *render_tree_iterator_init(void *render_tree);
struct node_t *render_tree_node_init();
void render_tree_node_free(struct node_t **node);
extern const void *render_tree_next_node(void *tree_iterator);
extern void render_tree_get_node_data(const void *current_node,
                                      struct node_t *node);
extern void render_tree_iterator_free(void *tree_iterator);
extern void render_tree_free(void *render_free);

struct node_text_t {
  char *value;
  char *font;
  float font_size;
  bool is_bold;
};

enum node_type_e { NODE_TYPE_ROOT = 0u, NODE_TYPE_TEXT };

struct node_t {
  enum node_type_e type;
  union data {
    bool root;               // NODE_TYPE_ROOT
    struct node_text_t text; // NODE_TYPE_TEXT
  } data;
};

struct node_t *render_tree_node_init() {
  struct node_t *node = malloc(sizeof(*node));
  if (!node)
    return NULL;

  node->type = NODE_TYPE_ROOT;
  node->data.root = true; // dummy value

  return node;
}

void render_tree_node_free(struct node_t **node) {
  free(*node);
  *node = NULL;
}

#endif
