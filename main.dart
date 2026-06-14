import 'package:flutter/material.dart';

void main() {
  runApp(const MaterialApp(
    debugShowCheckedModeBanner: false,
    home: ProsperaDummyWallet(),
  ));
}

class ProsperaDummyWallet extends StatelessWidget {
  const ProsperaDummyWallet({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: const Color(0xFF0B0E14),
      appBar: AppBar(
        title: const Text('PROSPERA MOBILE WALLET', style: TextStyle(color: Color(0xFF00FFCC), fontSize: 16, fontWeight: FontWeight.bold)),
        backgroundColor: const Color(0xFF151A22),
        centerTitle: true,
      ),
      body: Padding(
        padding: const EdgeInsets.all(20.0),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Container(
              padding: const EdgeInsets.all(20),
              decoration: BoxDecoration(
                color: const Color(0xFF151A22),
                borderRadius: BorderRadius.circular(16),
                border: Border.all(color: const Color(0xFF00FFCC)),
              ),
              child: const Column(
                children: [
                  Text("AVAILABLE BALANCE", style: TextStyle(color: Colors.grey, fontSize: 12)),
                  SizedBox(height: 10),
                  Text("1,200,000 PRSP", style: TextStyle(color: Color(0xFF00FFCC), fontSize: 32, fontWeight: FontWeight.bold)),
                  SizedBox(height: 15),
                  Text("WALLET ADDRESS:", style: TextStyle(color: Colors.grey, fontSize: 11)),
                  Text("prsp17ed3e8fe4fc3fa9aed0524c4b81731f7", style: TextStyle(color: Colors.white, fontSize: 11)),
                ],
              ),
            ),
            const SizedBox(height: 30),
            ElevatedButton(
              style: ElevatedButton.styleFrom(
                backgroundColor: const Color(0xFF00FFCC),
                foregroundColor: Colors.black,
                minimumSize: const Size.fromHeight(50),
              ),
              onPressed: () {},
              child: const Text('CREATE NEW WALLET', style: TextStyle(fontWeight: FontWeight.bold)),
            ),
          ],
        ),
      ),
    );
  }
}
