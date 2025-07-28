import os
from openai import OpenAI

client = OpenAI(api_key=os.environ["OPENAI_API_KEY"])

def read_code_files():
    code = ""
    for root, dirs, files in os.walk("src"):
        for f in files:
            if f.endswith(".rs"):
                with open(os.path.join(root, f)) as file:
                    code += f"# {f}\n" + file.read() + "\n\n"
    return code

prompt = f"""
You are an expert documentation generator.
Generate a detailed README.md from the following codebase:

{read_code_files()}
"""

response = client.chat.completions.create(
    model="gpt-4",
    messages=[
        {"role": "system", "content": "You are a helpful assistant that writes clear documentation."},
        {"role": "user", "content": prompt}
    ]
)

content = response.choices[0].message.content

os.makedirs("docs", exist_ok=True)
with open("docs/generated.md", "w") as f:
    f.write(content)
