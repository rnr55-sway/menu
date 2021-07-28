## Examples
 
```
cat menu-example.json | jq ".x = $(swaymsg -t get_outputs -r | jq '[.[] | select(.active?)][0] | .rect.width + .rect.x')" | cargo run
```