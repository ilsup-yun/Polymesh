./target/release/polymesh benchmark pallet -s 100 -r 5 -p=pallet_im_online -e=* --heap-pages 4096 --db-cache 512 --execution wasm --wasm-execution compiled --output  ./pallets/weights/src/ --template ./.maintain/frame-weight-template.hbs  >> data.txt 2>> log.txt
./target/release/polymesh benchmark pallet -s 100 -r 5 -p=pallet_compliance_manager -e=* --heap-pages 4096 --db-cache 512 --execution wasm --wasm-execution compiled --output  ./pallets/weights/src/ --template ./.maintain/frame-weight-template.hbs  >> data.txt 2>> log.txt
./target/release/polymesh benchmark pallet -s 100 -r 5 -p=pallet_sto -e=* --heap-pages 4096 --db-cache 512 --execution wasm --wasm-execution compiled --output  ./pallets/weights/src/ --template ./.maintain/frame-weight-template.hbs  >> data.txt 2>> log.txt
./target/release/polymesh benchmark pallet -s 100 -r 5 -p=pallet_relayer -e=* --heap-pages 4096 --db-cache 512 --execution wasm --wasm-execution compiled --output  ./pallets/weights/src/ --template ./.maintain/frame-weight-template.hbs  >> data.txt 2>> log.txt
./target/release/polymesh benchmark pallet -s 100 -r 5 -p=pallet_identity -e=* --heap-pages 4096 --db-cache 512 --execution wasm --wasm-execution compiled --output  ./pallets/weights/src/ --template ./.maintain/frame-weight-template.hbs  >> data.txt 2>> log.txt
./target/release/polymesh benchmark pallet -s 100 -r 5 -p=pallet_committee -e=* --heap-pages 4096 --db-cache 512 --execution wasm --wasm-execution compiled --output  ./pallets/weights/src/ --template ./.maintain/frame-weight-template.hbs  >> data.txt 2>> log.txt
./target/release/polymesh benchmark pallet -s 100 -r 5 -p=pallet_statistics -e=* --heap-pages 4096 --db-cache 512 --execution wasm --wasm-execution compiled --output  ./pallets/weights/src/ --template ./.maintain/frame-weight-template.hbs  >> data.txt 2>> log.txt
./target/release/polymesh benchmark pallet -s 100 -r 5 -p=pallet_protocol_fee -e=* --heap-pages 4096 --db-cache 512 --execution wasm --wasm-execution compiled --output  ./pallets/weights/src/ --template ./.maintain/frame-weight-template.hbs  >> data.txt 2>> log.txt
./target/release/polymesh benchmark pallet -s 100 -r 5 -p=pallet_group -e=* --heap-pages 4096 --db-cache 512 --execution wasm --wasm-execution compiled --output  ./pallets/weights/src/ --template ./.maintain/frame-weight-template.hbs  >> data.txt 2>> log.txt
./target/release/polymesh benchmark pallet -s 100 -r 5 -p=pallet_checkpoint -e=* --heap-pages 4096 --db-cache 512 --execution wasm --wasm-execution compiled --output  ./pallets/weights/src/ --template ./.maintain/frame-weight-template.hbs  >> data.txt 2>> log.txt
./target/release/polymesh benchmark pallet -s 100 -r 5 -p=frame_system -e=* --heap-pages 4096 --db-cache 512 --execution wasm --wasm-execution compiled --output  ./pallets/weights/src/ --template ./.maintain/frame-weight-template.hbs  >> data.txt 2>> log.txt
