/*!

```c
#include "src/Chat.h"
#include "src/GameStructs.h"

#ifdef CC_BUILD_WIN
    #define CC_API __declspec(dllimport)
    #define CC_VAR __declspec(dllimport)
    #define EXPORT __declspec(dllexport)
#else
    #define CC_API
    #define CC_VAR
    #define EXPORT __attribute__((visibility("default")))
#endif

static void TestPlugin_Init(void) {
        String msg = String_FromConst("Hello world!");
        Chat_Add(&msg);
}

EXPORT int Plugin_ApiVersion = 1;
EXPORT struct IGameComponent Plugin_Component = { TestPlugin_Init };
```

```rust
#[no_mangle]
pub static Plugin_ApiVersion: c_int = 1;

#[no_mangle]
pub static Plugin_Component: IGameComponent = IGameComponent {
  ...
};

```


*/

mod os;

pub use crate::os::*;
