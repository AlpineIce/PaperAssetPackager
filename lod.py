import bpy

class PaperAssetPackagerLODPanel(bpy.types.Panel):
    bl_category = "Paper"
    bl_label = "Object LODs"
    bl_idname = "paper.asset_packager_lod_panel"
    bl_space_type = 'PROPERTIES'
    bl_region_type = 'WINDOW'
    bl_context = "data"

    @classmethod
    def poll(cls, context):
        object = context.object
        return (object != None) and (object.type == 'MESH')
    
    def draw(self, context):
        layout = self.layout

        layout.label(text="TODO LIST OF LODS")
        