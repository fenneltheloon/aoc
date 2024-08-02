# Plan for repeating 2char string

- Iterate through string, 0->len-2
- Slice string twice, 0-i, i-i+2, i+2-len
- Combine outside two slices
- Check to see if inside slice in the rest
  - Only need to go through half of the outside string?

# Plan for sandwich
- Iterate 0-len-2
- Check character 2 ahead for equality
