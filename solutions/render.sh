#!/usr/bin/env sh
R -q -e "rmarkdown::render('report.rmd', output_dir='/tmp')"
