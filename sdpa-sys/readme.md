# prerequired

* intel OneAPI Base kit(https://www.intel.com/content/www/us/en/developer/tools/oneapi/base-toolkit.html#gs.gtxufc) for MKL(BLAS, LAPACK)
* intel OneAPI HPC kit(https://www.intel.com/content/www/us/en/developer/tools/oneapi/hpc-toolkit.html#gs.gtxuh2) for MPI
* pkgconfig

# intel OneAPI enviroment variables setting
See here(https://www.intel.com/content/www/us/en/develop/documentation/get-started-with-intel-oneapi-hpc-windows/top/run-a-sample-project-using-the-command-line.html#run-a-sample-project-using-the-command-line).

```
# need to type terminal (on Windows)
cmd.exe "/K" '"C:\Program Files (x86)\Intel\oneAPI\setvars.bat" && powershell'
```