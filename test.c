#include <stddef.h>
#include <stdio.h>

extern void *render_tree_init();
extern size_t render_tree_next_node(const void *render_tree);

int main(void) {
  void *render_tree = render_tree_init();
  printf("%p\n", render_tree);
  printf("N Children: %zu\n", render_tree_next_node(render_tree));
  return 0;
}
