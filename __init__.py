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
    importlib.reload(lod)
    importlib.reload(package)
else:
    from .lod import PaperAssetPackagerLODPanel
    from .package import PackageOperator
    
import bpy

#----------UI PANEL----------#

class PaperAssetPackagerPanel(bpy.types.Panel):
    bl_category = "Paper"
    bl_label = "Paper Asset Packager"
    bl_idname = "paper.asset_packager_main_panel"
    bl_space_type = 'VIEW_3D'
    bl_region_type = 'UI'

    def draw(self, context):
        layout = self.layout

        scene = context.scene
        
        # Big render button
        layout.label(text="Package Scene:")
        row = layout.row()
        row.scale_y = 2.0
        row.operator("paper.package_operator")


#----------REGISTRATION----------#

def register():
    bpy.utils.register_class(PaperAssetPackagerLODPanel)
    bpy.utils.register_class(PackageOperator)
    bpy.utils.register_class(PaperAssetPackagerPanel)


def unregister():
    bpy.utils.unregister_class(PaperAssetPackagerPanel)
    bpy.utils.unregister_class(PackageOperator)
    bpy.utils.unregister_class(PaperAssetPackagerLODPanel)


if __name__ == "__main__":
    register()