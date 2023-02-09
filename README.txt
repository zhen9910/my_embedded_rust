

cargo rustc -- -C link-arg=--script=./linker.ld

arm-none-eabi-objdump -D target/armv7a-none-eabi/debug/my_embedded_rust | less

file kernel7.img

arm-none-eabi-objcopy -O binary target/armv7a-none-eabi/debug/my_embedded_rust ./kernel7.img