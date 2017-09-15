#include <stdio.h>
#include <dlfcn.h>

int main(void) {
  void* h = dlopen("./target/debug/libsimpleservo.dylib", RTLD_LOCAL);
  if (h) {
    char* (*version)() = dlsym(h, "servo_version");
    if (version) {
      printf("servo version: %s\n", version());
    } else {
      printf("Can't find symbol\n");
    }
  } else {
    printf("Can't find lib\n");
  }
  if (dlclose(h) != 0) {
    printf("Unable to close library: %s", dlerror());
  }
}
