import bpy

#----------UI PANEL----------#

class PAPER_ASSET_PACKAGER_PT_package_main_panel(bpy.types.Panel):
    bl_category = "Paper"
    bl_label = "Paper Asset Packager"
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

class PAPER_ASSET_PACKAGER_OT_package_operator(bpy.types.Operator):
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
        #warning buffer
        msg_buffer = ""

        #verify base models collection
        base_models_name = "Base Models"
        if base_models_name not in bpy.data.collections:
            bpy.ops.collection.create(name=base_models_name)
            bpy.context.scene.collection.children.link(bpy.data.collections[base_models_name])

            msg_buffer += "Base Model collection doesn't exist and has been created; "
        
        #verify model instances collection
        model_instances_name = "Model Instances"
        if model_instances_name not in bpy.data.collections:
            bpy.ops.collection.create(name=model_instances_name)
            bpy.context.scene.collection.children.link(bpy.data.collections[model_instances_name])

            msg_buffer += "Model Instances collection doesn't exist and has been created; "

        #print any warnings
        if len(msg_buffer): self.report({"WARNING"}, msg_buffer)
        
        self.get_unique_models(context)
        
        return {'FINISHED'}
    