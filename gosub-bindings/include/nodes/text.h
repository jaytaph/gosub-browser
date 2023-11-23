#ifndef GOSUB_API_NODES_TEXT_H
#define GOSUB_API_NODES_TEXT_H

#include <stdbool.h>

struct node_t;

struct node_text_t {
  char *value;
  char *font;
  float font_size;
  bool is_bold;
};

const char *render_tree_node_text_value(const struct node_t *node);
const char *render_tree_node_text_font(const struct node_t *node);
float render_tree_node_text_font_size(const struct node_t *node);
bool render_tree_node_text_bold(const struct node_t *node);

#endif
