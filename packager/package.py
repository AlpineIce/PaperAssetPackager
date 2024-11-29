import bpy
import mathutils

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

#----------MODEL DATA STRUCTURES----------#

#vertex data
class Vertex:
    position = [0, 0, 0]
    normal = [0, 0, 1]
    uv = [0, 0]


#Model -> LODs -> Meshes
class Mesh:
    vbo_offset = -1
    ibo_offset = -1
    mat_slot = -1


class LOD:
    meshes = []


class ModelData:
    lod_data = []
    vertex_data = []

#----------PACKAGE OPERATOR----------#

class PAPER_ASSET_PACKAGER_OT_package_operator(bpy.types.Operator):
    bl_idname = "paper.package_operator"
    bl_label = "Package Scene"
    
    @classmethod
    def poll(cls, context):
        return context.scene is not None
    
    def get_base_models(self, base_model_collection) -> ModelData:
        #return data
        model_datas = []
        
        #iterate objects in scene
        for object in base_model_collection.all_objects:
            #verify object is mesh type
            if object.type == "MESH":
                model_data = ModelData()

                #                                                           #
                #----------TODO HANDLE SMOOTH AND FLAT SHADING!!!!----------#
                #                                                           #

                #add vertex data to model_data
                for mesh_vertex in object.data.vertices:
                    vertex = Vertex()
                    vertex.position = mesh_vertex.co
                    vertex.normal = mesh_vertex.normal

                    #vertex.uv = mesh_vertex.
                    model_data.vertex_data.append(vertex)

                #get UV data
                uv = object.data.uv_layers.active.uv

                #iterate vertex groups and create map of them first
                groups = {}
                for group in object.vertex_groups:
                    #check if group is an LOD
                    if("LOD" in group.name):
                        split = group.name.split("LOD")
                        groups[int(split[1])] = group
                
                #then iterate polygons and associate them with their LOD and material
                total_index_size = 0
                for polygon in object.data.polygons:
                    indices = polygon.vertices
                    mat_slot = polygon.material_index
                    lod_slot = -1

                    total_index_size += len(indices)

                    #find LOD (idk if theres a better way to do this tbh)
                    for key, value in groups.items():
                        try:
                            weight = value.weight(indices[0]) #polygons are guaranteed to have the same LOD for every vertex (there shouldn't be any faces attaching LODs together)
                            if weight > 0.5:
                                lod_slot = key

                            break
                        except:
                            continue

                    #add model data if an lod slot exists
                    if lod_slot != -1:
                        for index in indices:
                            vertex = object.data.vertices[index]

                print(total_index_size)
                #model_data.lod_data[lod_slot].meshes[mat_slot]

                    #print(str(lod_slot) + " " + str(mat_slot))

                    
                    
    def get_model_instances(self, model_instance_collection):
        for object in model_instance_collection.all_objects:
            if object.type == "MESH":
                print("Model Instance")
    
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
        
        #get base models and model instances
        models = self.get_base_models(bpy.data.collections[base_models_name])
        model_instances = self.get_model_instances(bpy.data.collections[model_instances_name])
        
        return {'FINISHED'}
    