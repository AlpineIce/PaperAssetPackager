import bpy

#----------LOD PANEL----------#

class PaperAssetPackagerLODPanel(bpy.types.Panel):
    bl_label = "Object LODs"
    bl_idname = "PAPER_ASSET_PACKAGER_PT_lod_panel"
    bl_space_type = 'PROPERTIES'
    bl_region_type = 'WINDOW'
    bl_context = "data"

    @classmethod
    def poll(cls, context):
        object = context.object
        return (object != None) and (object.type == 'MESH') #only draw if selected object is a mesh
    
    def draw(self, context):
        layout = self.layout

        layout.label(text="TODO LIST OF LODS")

#----------LOD LIST----------#

class PaperAssetPackagerLODList(bpy.types.UIList):
    bl_label = "LOD List"
    bl_idname = "PAPER_ASSET_PACKAGER_UL_lod_list"

    def draw_item(context, layout, data, item, icon, active_data, active_property, *, index=0, flt_flag=0):
        scene = context.scene
        row = layout.row(align=True)
