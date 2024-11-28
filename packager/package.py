import bpy

#----------UI PANEL----------#

class PaperAssetPackagerPanel(bpy.types.Panel):
    bl_category = "Paper"
    bl_label = "Paper Asset Packager"
    bl_idname = "PAPER_ASSET_PACKAGER_PT_main_panel"
    bl_space_type = 'VIEW_3D'
    bl_region_type = 'UI'

    @classmethod
    def poll(cls, context):
        return True #always draw this panel

    def draw(self, context):
        layout = self.layout

        scene = context.scene
        
        # Big render button
        layout.label(text="Package Scene:")
        row = layout.row()
        row.scale_y = 2.0
        row.operator("paper.package_operator")

#----------PACKAGE OPERATOR----------#

class PackageOperator(bpy.types.Operator):
    bl_idname = "paper.package_operator"
    bl_label = "Package Scene"
    
    @classmethod
    def poll(cls, context):
        return context.scene is not None
    
    def get_unique_models(self, context):
        #list of unique objects
        unique_objects = []
        
        #iterate objects in scene
        for object in context.scene.objects:
            if object.type == "MESH" and object:
                print("yay")
    
    def execute(self, context):
        self.get_unique_models(context)
        
        return {'FINISHED'}