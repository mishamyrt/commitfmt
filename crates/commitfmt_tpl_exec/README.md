# Commitfmt Template Executor

The package contains a renderer of templates that contain shell commands.

## Example

Anything inside the double curly braces (`{{.*}}`) will be executed as a command and the template will display the result of that command.

For example, if I execute this template on my laptop:

```
{{ echo $USER  | tr 'a-z' 'A-Z' }} <{{ git config user.email }}>
```

It will return:

```
MISHAMYRT <misha@myrt.co>
```

## Usage

The package provides a single `render` function that returns a result with a rendering of the provided template.

```rust

fn main() {
  let template = "{{ echo $USER  | tr 'a-z' 'A-Z' }} <{{ git config user.email }}>";
  let rendered = commitfmt_tpl_exec::render(template).expect("Rendering failed");
  println!("{}", rendered);
}
```
