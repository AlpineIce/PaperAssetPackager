#include "../include/prb_import.hpp"

#include <iostream>

int main()
{
    const std::vector<PaperAssetPackager::ModelData> model_data = PaperAssetPackager::get_model_data("output.prb");

    return 0;
}