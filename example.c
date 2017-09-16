#include <stdio.h>
#include <dlfcn.h>

void* callback(char* name) {
  return 0;
}

int main(void) {

  void* h = dlopen("./target/debug/libsimpleservo.dylib", RTLD_LOCAL);

  if (!h) {
    printf("Can't find lib\n");
    return 1;
  }

  char* (*version)() = dlsym(h, "servo_version");
  void (*init)() = dlsym(h, "init");
  void (*ping)() = dlsym(h, "ping");

  if (!version) {
    printf("%s", "Can't find version symbol");
    return 1;
  }

  if (!init) {
    printf("%s", "Can't find init symbol");
    return 1;
  }

  if (!ping) {
    printf("%s", "Can't find ping symbol");
    return 1;
  }

  printf("%s", version());
  init();

  while(1) {
    ping();
  }

  if (dlclose(h) != 0) {
    printf("Unable to close library: %s", dlerror());
    return 1;
  }

  return 0;
}
