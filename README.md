<div align="center">
<pre>
╔══════════════════════════════════════════════════════════════════════╗
║                                                                      ║
║     _     ____  __  __       _____  _   _   ____  ___  _   _  _____  ║
║    / \   |  _ \ \ \/ /      | ____|| \ | | / ___||_ _|| \ | || ____| ║
║   / _ \  | |_) | \  /  ____ |  _|  |  \| || |  _  | | |  \| ||  _|   ║
║  / ___ \ |  _ <  /  \ |____|| |___ | |\  || |_| | | | | |\  || |___  ║
║ /_/   \_\|_| \_\/_/\_\      |_____||_| \_| \____||___||_| \_||_____| ║
║                                                                      ║
║  ------------------------------------------------------------------  ║

---

## Build
```bash

fasm Arx-Engine.asm Arx-Engine.o
ar rcs Arx-Engine.a Arx-Engine.o
fasm main.asm main.o

ld \
    --static \
    -e arx_start \
    --gc-sections \
    --build-id=none \
    -z noexecstack \
    --no-eh-frame-hdr \
    --hash-style=sysv \
    -z noseparate-code \
    -O2 \
    main.o Arx-Engine.a \
    -o Arx-Engine

ls -la Arx-Engine
ldd Arx-Engine || true
```

## License

MIT License

Copyright (c) 2026 Help From the Void Independent Systems

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
