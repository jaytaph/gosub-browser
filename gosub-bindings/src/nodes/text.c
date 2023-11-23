#include "text.h"
#include "nodes.h"

const char *render_tree_node_text_value(const struct node_t *node) {
  if (!node)
    return NULL;

  return (const char *)node->data.text.value;
}

const char *render_tree_node_text_font(const struct node_t *node) {
  if (!node)
    return NULL;

  return (const char *)node->data.text.font;
}

float render_tree_node_text_font_size(const struct node_t *node) {
  if (!node)
    return 0.0f;

  return node->data.text.font_size;
}

bool render_tree_node_text_bold(const struct node_t *node) {
  if (!node)
    return false;

  return node->data.text.is_bold;
}
