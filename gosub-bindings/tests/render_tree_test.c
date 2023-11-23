#include "gosub_api.h"
#include <assert.h>
#include <math.h>
#include <stdio.h>
#include <string.h>

int main() {
  void *render_tree = render_tree_init();
  assert(render_tree != NULL);

  void *tree_iterator = render_tree_iterator_init(render_tree);
  assert(tree_iterator != NULL);

  const void *current_node = render_tree_next_node(tree_iterator);
  assert(current_node != NULL);

  struct node_t *node_data = render_tree_node_init();
  assert(node_data != NULL);

  // <html>
  render_tree_get_node_data(current_node, node_data);
  assert(node_data->type == NODE_TYPE_ROOT);

  // <h1>
  current_node = render_tree_next_node(tree_iterator);
  render_tree_get_node_data(current_node, node_data);
  assert(node_data->type == NODE_TYPE_TEXT);
  assert(strcmp(node_data->data.text.value, "heading1") == 0);
  assert(strcmp(node_data->data.text.font, "Times New Roman") == 0);
  assert(fabsf(node_data->data.text.font_size - 37.0f) < 0.00001f);
  assert(node_data->data.text.is_bold == true);

  // end of iterator (frees final node)
  current_node = render_tree_next_node(tree_iterator);
  assert(current_node == NULL);

  // cleanup remaining memory
  render_tree_iterator_free(tree_iterator);
  render_tree_free(render_tree);
  render_tree_node_free(&node_data);

  return 0;
}
