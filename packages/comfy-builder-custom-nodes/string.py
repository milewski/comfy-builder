from typing_extensions import override

from comfy_api.latest import ComfyExtension, io

class StringConcatenate(io.ComfyNode):
    @classmethod
    def define_schema(cls):
        return io.Schema(
            node_id="StringConcatenate",
            display_name="Concatenate",
            category="utils/string",
            inputs=[
                io.String.Input("string_a", multiline=True),
                io.String.Input("string_b", multiline=True),
                io.String.Input("delimiter", multiline=False, default=""),
            ],
            outputs=[
                io.String.Output(),
            ]
        )

    @classmethod
    def execute(cls, string_a, string_b, delimiter):
        return io.NodeOutput(delimiter.join((string_a, string_b)))

class StringExtension(ComfyExtension):
    @override
    async def get_node_list(self) -> list[type[io.ComfyNode]]:
        return [
            StringConcatenate,
        ]

async def comfy_entrypoint() -> StringExtension:
    return StringExtension()