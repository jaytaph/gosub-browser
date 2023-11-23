#include "nodes.h"

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
