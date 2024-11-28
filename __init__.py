#----------ADDON INFO----------#

bl_info = {
    "name": "Paper Asset Packager",
    "description": "Custom asset packager for PaperRenderer example; packages models into a single binary blob.",
    "author": "AlpineIce",
    "version": (1, 0, 0),
    "blender": (4, 3, 0),
    "location": "View3D > UI",
    "warning": "",
    "wiki_url": "https://github.com/AlpineIce/PaperAssetPackager",
    "tracker_url": "my github url here/issues",
    "category": "Animation"
}

#----------LOAD OTHER FILES----------#

if "bpy" in locals():
    import importlib
    importlib.reload(packager)
else:
    from .packager import *

import bpy


#----------REGISTRATION----------#

classes = (
    package.PackageOperator,
    package.PaperAssetPackagerPanel,
    lod.PaperAssetPackagerLODPanel,
    lod.PaperAssetPackagerLODList
)

def register():
    for cls in classes:
        bpy.utils.register_class(cls)

def unregister():
    for cls in classes:
        bpy.utils.unregister_class(cls)


if __name__ == "__main__":
    register()