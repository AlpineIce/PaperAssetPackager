import bpy

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