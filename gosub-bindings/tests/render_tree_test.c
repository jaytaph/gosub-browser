#include "gosub_api.h"
#include <assert.h>
#include <math.h>
#include <stdio.h>
#include <string.h>

int main() {
  struct render_tree_t *render_tree = render_tree_init();
  assert(render_tree != NULL);

  const struct node_t *node = NULL;

  // <html>
  node = render_tree_next(render_tree);
  assert(node->type == NODE_TYPE_ROOT);

  // <h1>
  node = render_tree_next(render_tree);
  assert(node->type == NODE_TYPE_TEXT);
  assert(strcmp(node->data.text.value, "heading1") == 0);
  assert(strcmp(node->data.text.font, "Times New Roman") == 0);
  assert(fabsf(node->data.text.font_size - 37.0f) < 0.00001f);
  assert(node->data.text.is_bold == true);

  // end of iterator, last node is free'd
  node = render_tree_next(render_tree);
  assert(node == NULL);

  render_tree_free(&render_tree);

  printf("render_tree_test.c: All assertions passed\n");

  return 0;
}
