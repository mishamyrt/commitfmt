"""Shell command utils"""
import subprocess

class ShellError(subprocess.CalledProcessError):
    """Shell error raised when shell command fails"""

def shell(command: str, cwd: str = None) -> str:
    """Run a shell command and return output"""
    try:
        process = subprocess.run(
            command,
            shell=True,
            check=True,
            capture_output=True,
            text=True,
            cwd=cwd)
    except subprocess.CalledProcessError as exc:
        print(exc.output)
        raise ShellError(
            returncode=exc.returncode,
            cmd=command,
            output=exc.output,
            stderr=exc.stderr
        ) from exc
    if process.returncode != 0:
        print(process.stdout)
        print(process.stderr)
        raise ShellError(
            returncode=process.returncode,
            cmd=command,
            output=process.stdout,
            stderr=process.stderr
        )
    return process.stdout
