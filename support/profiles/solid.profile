id = '5dc59fa8-e965-45cb-a1da-e87d28713395'
name = 'Solid Color'
description = 'Shows just a solid color'
active_scripts = [
	'solid.lua',
	'shockwave.lua',
#	'impact.lua',
#   'dim-zone.lua',
 	'macros.lua',
#	'stats.lua',
]

[[config."Solid Color"]]
type = 'color'
name = 'color_background'
value = 0xffff0000
default = 0xffff0000

# dim a specific zone, e.g. if the mouse LEDs are too bright
[[config."Dim Zone"]]
type = 'int'
name = 'zone_start'
value = 144
default = 144

[[config."Dim Zone"]]
type = 'int'
name = 'zone_end'
value = 180
default = 180

[[config."Dim Zone"]]
type = 'float'
name = 'opacity'
value = 0.95
default = 0.95