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

  render_tree_free(&render_tree);

  printf("render_tree_test.c: All assertions passed\n");

  return 0;
}
