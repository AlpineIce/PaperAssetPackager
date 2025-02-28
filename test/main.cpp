#include "../include/prb_import.hpp"

#include <iostream>

int main()
{
    std::unique_ptr<std::ifstream> file = PaperAssetPackager::get_paper_binary("../../paper_asset_packager/output/output.prb");
    
    if(file)
    {
        const std::unordered_map<std::string, uint64_t> model_names = PaperAssetPackager::get_model_name_indices(*file);
        const std::vector<PaperAssetPackager::ModelData> model_data = PaperAssetPackager::get_all_model_data(*file);

        int a = 0; //useful breakpoint
    }

    return 0;
}