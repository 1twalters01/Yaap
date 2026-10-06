from strictdoc.core.project_config import ProjectConfig


def create_config() -> ProjectConfig:
    return ProjectConfig(
        project_title="YAAP",
        project_features=[
            # Stable features.
            "TABLE_SCREEN",
            "TRACEABILITY_SCREEN",
            "DEEP_TRACEABILITY_SCREEN",
            "SEARCH",
            "TRACEABILITY_MATRIX_SCREEN",
            "REQUIREMENT_TO_SOURCE_TRACEABILITY",
            # "MATHJAX"

            # Experimental features.
            # "PROJECT_STATISTICS_SCREEN",
            # "TREE_MAP_SCREEN",
            # "REQIF",
            # "HTML2PDF",
            # "DIFF",
        ],
        source_root_path="../..",
        include_doc_paths=[
            "business_requirements.sdoc",
        ],
        include_source_paths=[
            "/libs/**",
            "/apps/**",
            "/frontends/**",
        ],
    )
