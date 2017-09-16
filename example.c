#include <stdio.h>
#include <dlfcn.h>

void* callback(char* name) {
  return 0;
}

int main(void) {
  void* h = dlopen("./target/debug/libsimpleservo.dylib", RTLD_LOCAL);
  if (h) {
    void (*init)(void *(char *)) = dlsym(h, "init");
    if (init) {
      init(callback);
    } else {
      printf("Can't find symbol\n");
    }
  } else {
    printf("Can't find lib\n");
  }
  if (dlclose(h) != 0) {
    printf("Unable to close library: %s", dlerror());
  }
  while(1) {
  }
}
