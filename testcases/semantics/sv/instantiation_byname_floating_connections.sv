module M1 (input in1_1, in1_2, output out1_1, out1_2);
endmodule

module M2(input in2_1, in2_2, output out2_1, out2_2);
    M1 i1 (.in1_1(in2_1), .in1_2(), .out1_1(out2_1), .out1_2());
    M1 i2 (.in1_1(w1), .in1_2(w1), .out1_1(w2), .out1_2());
endmodule

module M3 (input in3_1, in3_2, in3_3, output out3_1, out3_2);
    M2 i2 (.in2_1(in3_1), .in2_2(in3_2), .out2_1(out3_1), .out2_2());
endmodule

