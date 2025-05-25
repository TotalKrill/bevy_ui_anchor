.FORCE: 

readme: .FORCE
	cat README_start.md > README.md
	cat examples/simple.rs >> README.md
	cat README_end.md >> README.md
	

