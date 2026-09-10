# ISO C Memory Safety & Defect Prevention Reference

1. **Explicit Bounds**: Every pointer passed into a function representing an array or buffer must have its element count or byte capacity passed explicitly in the adjacent parameter.
2. **Immediate NULL Verification**: Check pointer immediately after allocation:
   ```c
   void* ptr = malloc(size);
   if (ptr == NULL) {
       return ERR_OUT_OF_MEMORY;
   }
   ```
3. **Double Free / UAF Defense**: Set pointer to NULL immediately after freeing:
   ```c
   free(ptr);
   ptr = NULL;
   ```
4. **Integer Overflow Checking**: Verify integer arithmetic before allocating memory:
   ```c
   if (count > SIZE_MAX / sizeof(Element)) {
       return ERR_OVERFLOW;
   }
   ```
